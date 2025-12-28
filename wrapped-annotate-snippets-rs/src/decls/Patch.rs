macro_rules! deps {
    () => {
        Snippet!();
    };
}

macro_rules! Patch {
    () => {
        deps!();
        # [doc = " Suggested edit to the [`Snippet`]"] # [doc = ""] # [doc = " See [`Snippet::patch`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/multi_suggestion.rs")] # [doc = " ```"] # [doc = ""] # [doc = include_str ! ("../examples/multi_suggestion.svg")] # [derive (Clone , Debug)] pub struct Patch < 'a > { pub (crate) span : Range < usize > , pub (crate) replacement : Cow < 'a , str > , }
    };
}

Patch!();