macro_rules! deps {
    () => {
        CasingStyle!();
        Sp!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl CasingStyle { fn from_lit (name : & LitStr) -> Result < Sp < Self > , syn :: Error > { use self :: CasingStyle :: { Camel , Kebab , Lower , Pascal , ScreamingSnake , Snake , Upper , Verbatim , } ; let normalized = name . value () . to_upper_camel_case () . to_lowercase () ; let cs = | kind | Sp :: new (kind , name . span ()) ; let s = match normalized . as_ref () { "camel" | "camelcase" => cs (Camel) , "kebab" | "kebabcase" => cs (Kebab) , "pascal" | "pascalcase" => cs (Pascal) , "screamingsnake" | "screamingsnakecase" => cs (ScreamingSnake) , "snake" | "snakecase" => cs (Snake) , "lower" | "lowercase" => cs (Lower) , "upper" | "uppercase" => cs (Upper) , "verbatim" | "verbatimcase" => cs (Verbatim) , s => abort ! (name , "unsupported casing: `{s}`") , } ; Ok (s) } }
    };
}

impl_72!();