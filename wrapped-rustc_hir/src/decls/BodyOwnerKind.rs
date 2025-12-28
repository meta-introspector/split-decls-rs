macro_rules! deps {
    () => {
        Closure!();
    };
}

macro_rules! BodyOwnerKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub enum BodyOwnerKind { # [doc = " Functions and methods."] Fn , # [doc = " Closures"] Closure , # [doc = " Constants and associated constants, also including inline constants."] Const { inline : bool } , # [doc = " Initializer of a `static` item."] Static (Mutability) , # [doc = " Fake body for a global asm to store its const-like value types."] GlobalAsm , }
    };
}

BodyOwnerKind!()