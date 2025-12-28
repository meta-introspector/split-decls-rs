macro_rules! deps {
    () => {
        Args!();
        ArgAction!();
        Str!();
        StyledStr!();
    };
}

macro_rules! PossibleValue {
    () => {
        deps!();
        # [doc = " A possible value of an argument."] # [doc = ""] # [doc = " This is used for specifying [possible values] of [Args]."] # [doc = ""] # [doc = " See also [`PossibleValuesParser`][crate::builder::PossibleValuesParser]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required"] # [doc = " to [hide] single values from help messages and shell completions or to attach [help] to"] # [doc = " possible values."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Arg, builder::PossibleValue, ArgAction};"] # [doc = " let cfg = Arg::new(\"config\")"] # [doc = "     .action(ArgAction::Set)"] # [doc = "     .value_name(\"FILE\")"] # [doc = "     .value_parser(["] # [doc = "         PossibleValue::new(\"fast\"),"] # [doc = "         PossibleValue::new(\"slow\").help(\"slower than fast\"),"] # [doc = "         PossibleValue::new(\"secret speed\").hide(true)"] # [doc = "     ]);"] # [doc = " ```"] # [doc = ""] # [doc = " [Args]: crate::Arg"] # [doc = " [possible values]: crate::builder::ValueParser::possible_values"] # [doc = " [hide]: PossibleValue::hide()"] # [doc = " [help]: PossibleValue::help()"] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct PossibleValue { name : Str , help : Option < StyledStr > , aliases : Vec < Str > , hide : bool , }
    };
}

PossibleValue!();