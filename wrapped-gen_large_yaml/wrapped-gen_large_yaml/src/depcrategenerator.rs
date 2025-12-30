// Generated macro for Generator (struct)
macro_rules! DepcrateGenerator {
() => {
// Module: crate
// Provides: {"Generator"}
// Dependencies: {}
# [doc = " YAML Generator."] struct Generator { # [doc = " The RNG state."] # [doc = ""] # [doc = " We don't need to be cryptographically secure. [`SmallRng`] also implements the"] # [doc = " [`SeedableRng`] trait, allowing runs to be predictable."] rng : SmallRng , # [doc = " The stack of indentations."] indents : Vec < usize > , }
};
}
