macro_rules! Styles {
    () => {
        # [doc = " Terminal styling definitions"] # [doc = ""] # [doc = " See also [`Command::styles`][crate::Command::styles]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " clap v3 styling"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::builder::styling::*;"] # [doc = " let styles = Styles::styled()"] # [doc = "     .header(AnsiColor::Yellow.on_default())"] # [doc = "     .usage(AnsiColor::Green.on_default())"] # [doc = "     .literal(AnsiColor::Green.on_default())"] # [doc = "     .placeholder(AnsiColor::Green.on_default());"] # [doc = " ```"] # [derive (Clone , Debug)] # [allow (missing_copy_implementations)] pub struct Styles { header : Style , error : Style , usage : Style , literal : Style , placeholder : Style , valid : Style , invalid : Style , context : Style , context_value : Option < Style > , }
    };
}

Styles!()