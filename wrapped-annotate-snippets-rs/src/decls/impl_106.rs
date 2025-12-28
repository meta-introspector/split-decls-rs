macro_rules! deps {
    () => {
        Report!();
        Renderer!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Renderer { # [doc = " Render a diagnostic [`Report`]"] pub fn render (& self , groups : Report < '_ >) -> String { render :: render (self , groups) } }
    };
}

impl_106!();