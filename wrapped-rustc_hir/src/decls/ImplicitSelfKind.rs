macro_rules! ImplicitSelfKind {
    () => {
        # [doc = " Represents what type of implicit self a function has, if any."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum ImplicitSelfKind { # [doc = " Represents a `fn x(self);`."] Imm , # [doc = " Represents a `fn x(mut self);`."] Mut , # [doc = " Represents a `fn x(&self);`."] RefImm , # [doc = " Represents a `fn x(&mut self);`."] RefMut , # [doc = " Represents when a function does not have a self argument or"] # [doc = " when a function has a `self: X` argument."] None , }
    };
}

ImplicitSelfKind!();