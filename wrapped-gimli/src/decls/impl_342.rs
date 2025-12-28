macro_rules! deps {
    () => {
        Reader!();
        Abbreviation!();
        Result!();
        Abbreviations!();
        Error!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl Abbreviations { # [doc = " Construct a new, empty set of abbreviations."] fn empty () -> Abbreviations { Abbreviations { vec : Vec :: new () , map : btree_map :: BTreeMap :: new () , } } # [doc = " Insert an abbreviation into the set."] # [doc = ""] # [doc = " Returns `Ok` if it is the first abbreviation in the set with its code,"] # [doc = " `Err` if the code is a duplicate and there already exists an"] # [doc = " abbreviation in the set with the given abbreviation's code."] fn insert (& mut self , abbrev : Abbreviation) -> :: core :: result :: Result < () , () > { let code_usize = abbrev . code as usize ; if code_usize as u64 == abbrev . code { if code_usize - 1 < self . vec . len () { return Err (()) ; } else if code_usize - 1 == self . vec . len () { if ! self . map . is_empty () && self . map . contains_key (& abbrev . code) { return Err (()) ; } else { self . vec . push (abbrev) ; return Ok (()) ; } } } match self . map . entry (abbrev . code) { btree_map :: Entry :: Occupied (_) => Err (()) , btree_map :: Entry :: Vacant (entry) => { entry . insert (abbrev) ; Ok (()) } } } # [doc = " Get the abbreviation associated with the given code."] # [inline] pub fn get (& self , code : u64) -> Option < & Abbreviation > { if let Ok (code) = usize :: try_from (code) { let index = code . checked_sub (1) ? ; if index < self . vec . len () { return Some (& self . vec [index]) ; } } self . map . get (& code) } # [doc = " Parse a series of abbreviations, terminated by a null abbreviation."] fn parse < R : Reader > (input : & mut R) -> Result < Abbreviations > { let mut abbrevs = Abbreviations :: empty () ; while let Some (abbrev) = Abbreviation :: parse (input) ? { if abbrevs . insert (abbrev) . is_err () { return Err (Error :: DuplicateAbbreviationCode) ; } } Ok (abbrevs) } }
    };
}

impl_342!();