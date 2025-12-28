macro_rules! deps {
    () => {
        Effects!();
        Color!();
    };
}

macro_rules! Style {
    () => {
        deps!();
        # [doc = " ANSI Text styling"] # [doc = ""] # [doc = " You can print a `Style` to render the corresponding ANSI code."] # [doc = " Using the alternate flag `#` will render the ANSI reset code, if needed."] # [doc = " Together, this makes it convenient to render styles using inline format arguments."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style = anstyle::Style::new().bold();"] # [doc = ""] # [doc = " let value = 42;"] # [doc = " println!(\"{style}{value}{style:#}\");"] # [doc = " ```"] # [derive (Copy , Clone , Default , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Style { fg : Option < crate :: Color > , bg : Option < crate :: Color > , underline : Option < crate :: Color > , effects : crate :: Effects , }
    };
}

Style!()