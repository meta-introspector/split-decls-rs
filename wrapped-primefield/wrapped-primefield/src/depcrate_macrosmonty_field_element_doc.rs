// Generated macro for monty_field_element_doc (macro)
macro_rules! Depcrate_macrosmonty_field_element_doc {
() => {
// Module: crate::macros
// Provides: {"monty_field_element_doc"}
// Dependencies: {}
# [doc = " Write documentation for a field element type."] # [doc (hidden)] # [macro_export] # [rustfmt :: skip] macro_rules ! monty_field_element_doc { ($ about : expr) => { concat ! ($ about , "\n\n" , "# Trait impls\n" , "\n" , "Much of the important functionality is provided by traits from the [`ff`] crate:\n" , "\n" , "- [`Field`] represents elements of finite fields and provides:\n" , "  - [`Field::random`] generate a random field element\n" , "  - `double`, `square`, and `invert` operations\n" , "  - Bounds for [`Add`], [`Sub`], [`Mul`], and [`Neg`] (and `*Assign` equivalents)\n" , "  - Bounds for [`ConditionallySelectable`] from the `subtle` crate\n" , "- [`PrimeField`] represents elements of prime fields and provides:\n" , "  - `from_repr`/`to_repr` for converting field elements from/to big integers.\n" , "  - `MULTIPLICATIVE_GENERATOR` and `ROOT_OF_UNITY` constants.\n" , "- [`PrimeFieldBits`] operations over field elements represented as bits " , "  (requires `bits` feature)\n" , "\n" , "Please see the documentation for the relevant traits for more information.\n") } ; }
};
}
