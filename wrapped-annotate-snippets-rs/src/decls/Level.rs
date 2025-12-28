macro_rules! deps {
    () => {
        LevelInner!();
    };
}

macro_rules! Level {
    () => {
        deps!();
        # [doc = " Severity level for [`Title`]s and [`Message`]s"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use annotate_snippets::*;"] # [doc = " let report = &["] # [doc = "     Level::ERROR.primary_title(\"mismatched types\").id(\"E0308\")"] # [doc = "         .element(Level::NOTE.message(\"expected reference\")),"] # [doc = "     Group::with_title("] # [doc = "         Level::HELP.secondary_title(\"function defined here\")"] # [doc = "     ),"] # [doc = " ];"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct Level < 'a > { pub (crate) name : Option < Option < Cow < 'a , str > > > , pub (crate) level : LevelInner , }
    };
}

Level!()