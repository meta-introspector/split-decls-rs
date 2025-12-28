macro_rules! truecolor {
    () => {
        # [doc = " Check [COLORTERM] for truecolor support"] # [doc = ""] # [doc = " [COLORTERM]: https://github.com/termstandard/colors"] # [inline] pub fn truecolor () -> bool { let value = std :: env :: var_os ("COLORTERM") ; let value = value . as_deref () . unwrap_or_default () ; value == "truecolor" || value == "24bit" }
    };
}

truecolor!()