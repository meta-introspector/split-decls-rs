macro_rules! deps {
    () => {
        Group!();
        Level!();
        Id!();
    };
}

macro_rules! Title {
    () => {
        deps!();
        # [doc = " A title that introduces a [`Group`], describing the main point"] # [doc = ""] # [doc = " To create a `Title`, see [`Level::primary_title`] or [`Level::secondary_title`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use annotate_snippets::*;"] # [doc = " let report = &["] # [doc = "     Group::with_title("] # [doc = "         Level::ERROR.primary_title(\"mismatched types\").id(\"E0308\")"] # [doc = "     ),"] # [doc = "     Group::with_title("] # [doc = "         Level::HELP.secondary_title(\"function defined here\")"] # [doc = "     ),"] # [doc = " ];"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Title < 'a > { pub (crate) level : Level < 'a > , pub (crate) id : Option < Id < 'a > > , pub (crate) text : Cow < 'a , str > , pub (crate) allows_styling : bool , }
    };
}

Title!()