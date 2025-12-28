macro_rules! CasingStyle {
    () => {
        # [doc = " Defines the casing for the attributes long representation."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum CasingStyle { # [doc = " Indicate word boundaries with uppercase letter, excluding the first word."] Camel , # [doc = " Keep all letters lowercase and indicate word boundaries with hyphens."] Kebab , # [doc = " Indicate word boundaries with uppercase letter, including the first word."] Pascal , # [doc = " Keep all letters uppercase and indicate word boundaries with underscores."] ScreamingSnake , # [doc = " Keep all letters lowercase and indicate word boundaries with underscores."] Snake , # [doc = " Keep all letters lowercase and remove word boundaries."] Lower , # [doc = " Keep all letters uppercase and remove word boundaries."] Upper , # [doc = " Use the original attribute name defined in the code."] Verbatim , }
    };
}

CasingStyle!()