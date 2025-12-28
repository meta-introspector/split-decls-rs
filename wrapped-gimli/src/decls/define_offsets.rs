macro_rules! deps {
    () => {
        BaseId!();
    };
}

macro_rules! define_offsets {
    () => {
        deps!();
        macro_rules ! define_offsets { ($ offsets : ident : $ id : ident => $ offset : ident , $ off_doc : expr) => { # [doc =$ off_doc] # [derive (Debug)] pub struct $ offsets { base_id : BaseId , offsets : Vec <$ offset >, } impl $ offsets { # [doc = " Return an empty list of offsets."] # [inline] pub fn none () -> Self { $ offsets { base_id : BaseId :: default () , offsets : Vec :: new () , } } # [doc = " Get the offset"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `id` is invalid."] # [inline] pub fn get (& self , id : $ id) -> $ offset { debug_assert_eq ! (self . base_id , id . base_id) ; self . offsets [id . index] } # [doc = " Return the number of offsets."] # [inline] pub fn count (& self) -> usize { self . offsets . len () } } } ; }
    };
}

define_offsets!()