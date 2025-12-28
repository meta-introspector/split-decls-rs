macro_rules! deps {
    () => {
        Signature!();
        IdentityRef!();
        Identity!();
        SignatureRef!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use crate :: { Identity , IdentityRef , Signature , SignatureRef } ; impl Identity { # [doc = " Borrow this instance as immutable"] pub fn to_ref (& self) -> IdentityRef < '_ > { IdentityRef { name : self . name . as_ref () , email : self . email . as_ref () , } } } impl From < IdentityRef < '_ > > for Identity { fn from (other : IdentityRef < '_ >) -> Identity { let IdentityRef { name , email } = other ; Identity { name : name . to_owned () , email : email . to_owned () , } } } impl < 'a > From < & 'a Identity > for IdentityRef < 'a > { fn from (other : & 'a Identity) -> IdentityRef < 'a > { other . to_ref () } } impl From < Signature > for Identity { fn from (Signature { name , email , time : _ } : Signature) -> Self { Identity { name , email } } } impl < 'a > From < SignatureRef < 'a > > for IdentityRef < 'a > { fn from (SignatureRef { name , email , time : _ } : SignatureRef < 'a >) -> Self { IdentityRef { name , email } } } }
    };
}

impls!();