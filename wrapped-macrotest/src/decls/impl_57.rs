macro_rules! deps {
    () => {
        ExpandedTest!();
        ExpansionOutcome!();
        Project!();
        Result!();
        ExpansionBehavior!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl ExpandedTest { pub fn run < I , S > (& self , project : & Project , expansion_behavior : ExpansionBehavior , args : & Option < I > ,) -> Result < ExpansionOutcome > where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { let (success , output_bytes) = cargo :: expand (project , & self . name , args) ? ; if ! success { return Ok (ExpansionOutcome :: ExpandError (output_bytes)) ; } let file_stem = self . test . file_stem () . expect ("no file stem") . to_string_lossy () . into_owned () ; let mut expanded = self . test . clone () ; expanded . pop () ; let expanded = & expanded . join (format ! ("{}.{}" , file_stem , EXPANDED_RS_SUFFIX)) ; let output = normalize_expansion (& output_bytes) ; if ! expanded . exists () { if let ExpansionBehavior :: ExpectFiles = expansion_behavior { return Ok (ExpansionOutcome :: NoExpandedFileFound) ; } std :: fs :: write (expanded , output) ? ; return Ok (ExpansionOutcome :: Update) ; } let expected_expansion_bytes = std :: fs :: read (expanded) ? ; let expected_expansion = String :: from_utf8_lossy (& expected_expansion_bytes) ; let same = output . lines () . eq (expected_expansion . lines ()) ; if ! same && project . overwrite { if let ExpansionBehavior :: ExpectFiles = expansion_behavior { return Ok (ExpansionOutcome :: NoExpandedFileFound) ; } std :: fs :: write (expanded , output) ? ; return Ok (ExpansionOutcome :: Update) ; } Ok (if same { ExpansionOutcome :: Same } else { let output_bytes = output . into_bytes () ; ExpansionOutcome :: Different (expected_expansion_bytes , output_bytes) }) } }
    };
}

impl_57!()