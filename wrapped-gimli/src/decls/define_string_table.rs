macro_rules! deps {
    () => {
        Writer!();
        BaseId!();
        FnvIndexSet!();
        Result!();
    };
}

macro_rules! define_string_table {
    () => {
        deps!();
        macro_rules ! define_string_table { ($ name : ident , $ id : ident , $ section : ident , $ offset : ident , $ docs : expr) => { # [doc =$ docs] # [derive (Debug , Default)] pub struct $ name { base_id : BaseId , strings : FnvIndexSet < Vec < u8 >>, offsets : Vec <$ offset >, len : usize , } impl $ name { # [doc = " Add a string to the string table and return its id."] # [doc = ""] # [doc = " If the string already exists, then return the id of the existing string."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `bytes` contains a null byte."] pub fn add < T > (& mut self , bytes : T) -> $ id where T : Into < Vec < u8 >>, { let bytes = bytes . into () ; assert ! (! bytes . contains (& 0)) ; let len = bytes . len () ; let (index , inserted) = self . strings . insert_full (bytes) ; if inserted { self . offsets . push ($ offset (self . len)) ; self . len += len + 1 ; } $ id :: new (self . base_id , index) } # [doc = " Return the number of strings in the table."] # [inline] pub fn count (& self) -> usize { self . strings . len () } # [doc = " Get a reference to a string in the table."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `id` is invalid."] pub fn get (& self , id : $ id) -> & [u8] { debug_assert_eq ! (self . base_id , id . base_id) ; self . strings . get_index (id . index) . map (Vec :: as_slice) . unwrap () } # [doc = " Get the offset of a string in the table."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `id` is invalid."] pub fn offset (& self , id : $ id) -> $ offset { debug_assert_eq ! (self . base_id , id . base_id) ; self . offsets [id . index] } # [doc = " Write the string table to the `.debug_str` section."] # [doc = ""] # [doc = " Returns the offsets at which the strings are written."] pub fn write < W : Writer > (& self , w : & mut $ section < W >) -> Result < () > { for bytes in self . strings . iter () { w . write (bytes) ?; w . write_u8 (0) ?; } Ok (()) } } } ; }
    };
}

define_string_table!();