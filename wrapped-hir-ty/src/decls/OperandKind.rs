macro_rules! deps {
    () => {
        Place!();
    };
}

macro_rules! OperandKind {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum OperandKind < 'db > { # [doc = " Creates a value by loading the given place."] # [doc = ""] # [doc = " Before drop elaboration, the type of the place must be `Copy`. After drop elaboration there"] # [doc = " is no such requirement."] Copy (Place < 'db >) , # [doc = " Creates a value by performing loading the place, just like the `Copy` operand."] # [doc = ""] # [doc = " This *may* additionally overwrite the place with `uninit` bytes, depending on how we decide"] # [doc = " in [UCG#188]. You should not emit MIR that may attempt a subsequent second load of this"] # [doc = " place without first re-initializing it."] # [doc = ""] # [doc = " [UCG#188]: https://github.com/rust-lang/unsafe-code-guidelines/issues/188"] Move (Place < 'db >) , # [doc = " Constants are already semantically values, and remain unchanged."] Constant { konst : Const < 'db > , ty : Ty < 'db > } , # [doc = " NON STANDARD: This kind of operand returns an immutable reference to that static memory. Rustc"] # [doc = " handles it with the `Constant` variant somehow."] Static (StaticId) , }
    };
}

OperandKind!();