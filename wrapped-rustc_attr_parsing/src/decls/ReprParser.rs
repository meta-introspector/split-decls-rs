macro_rules! ReprParser {
    () => {
        # [doc = " Parse #[repr(...)] forms."] # [doc = ""] # [doc = " Valid repr contents: any of the primitive integral type names (see"] # [doc = " `int_type_of_word`, below) to specify enum discriminant type; `C`, to use"] # [doc = " the same discriminant size that the corresponding C enum would or C"] # [doc = " structure layout, `packed` to remove padding, and `transparent` to delegate representation"] # [doc = " concerns to the only non-ZST field."] pub (crate) struct ReprParser ;
    };
}

ReprParser!()