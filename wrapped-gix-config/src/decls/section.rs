macro_rules! deps {
    () => {
        Header!();
        Event!();
        Key!();
        Comment!();
        Section!();
    };
}

macro_rules! section {
    () => {
        deps!();
        mod section { use std :: borrow :: Cow ; use bstr :: BStr ; use crate :: parse :: { section , section :: Header , Comment , Event , Events , Section } ; # [test] # [cfg (target_pointer_width = "64")] fn size_of_events () { assert_eq ! (std :: mem :: size_of ::< Section <'_ >> () , 96 , "this value should only ever decrease") ; assert_eq ! (std :: mem :: size_of ::< Events <'_ >> () , 616) ; assert_eq ! (std :: mem :: size_of ::< Event <'_ >> () , 72) ; assert_eq ! (std :: mem :: size_of ::< Header <'_ >> () , 72) ; assert_eq ! (std :: mem :: size_of ::< Comment <'_ >> () , 32) ; assert_eq ! (std :: mem :: size_of ::< Option < Cow <'_ , BStr >>> () , 24) ; assert_eq ! (std :: mem :: size_of ::< section :: Name <'_ >> () , 24) ; assert_eq ! (std :: mem :: size_of ::< section :: ValueName <'_ >> () , 24) ; } mod header { mod unvalidated { use crate :: parse :: section :: unvalidated :: Key ; # [test] fn section_name_only () { assert_eq ! (Key :: parse ("core") . unwrap () , Key { section_name : "core" , subsection_name : None }) ; } # [test] fn section_name_and_subsection () { assert_eq ! (Key :: parse ("core.bare") . unwrap () , Key { section_name : "core" , subsection_name : Some ("bare" . into ()) }) ; } # [test] fn section_name_and_subsection_with_separators () { assert_eq ! (Key :: parse ("remote.https:///home/user.git") . unwrap () , Key { section_name : "remote" , subsection_name : Some ("https:///home/user.git" . into ()) }) ; } } mod write_to { use std :: borrow :: Cow ; use crate :: parse :: section ; fn header (name : & str , subsection : impl Into < Option < (& 'static str , & 'static str) > >) -> section :: Header < '_ > { let name = section :: Name (Cow :: Borrowed (name . into ())) ; if let Some ((separator , subsection_name)) = subsection . into () { section :: Header { name , separator : Some (Cow :: Borrowed (separator . into ())) , subsection_name : Some (Cow :: Borrowed (subsection_name . into ())) , } } else { section :: Header { name , separator : None , subsection_name : None , } } } # [test] fn legacy_subsection_format_does_not_use_escapes () { let invalid = header ("invalid" , Some (("." , r#"\ ""#))) ; assert_eq ! (invalid . to_bstring () , r#"[invalid.\ "]"# , "no escaping happens for legacy subsections") ; assert ! (invalid . is_legacy ()) ; } # [test] fn subsections_escape_two_characters_only () { let invalid = header ("invalid" , Some ((" " , "\\ \"\npost newline"))) ; assert_eq ! (invalid . to_bstring () , "[invalid \"\\\\ \\\"\npost newline\"]" , "newlines are actually invalid in subsection, but they are possible due to unvalidated instance creation") ; assert ! (! invalid . is_legacy ()) ; } } } }
    };
}

section!()