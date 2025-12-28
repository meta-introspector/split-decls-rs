macro_rules! deps {
    () => {
        RefSpec!();
        RefSpecRef!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use std :: { cmp :: Ordering , hash :: { Hash , Hasher } , } ; use crate :: { RefSpec , RefSpecRef } ; impl From < RefSpecRef < '_ > > for RefSpec { fn from (v : RefSpecRef < '_ >) -> Self { v . to_owned () } } impl Hash for RefSpec { fn hash < H : Hasher > (& self , state : & mut H) { self . to_ref () . hash (state) ; } } impl Hash for RefSpecRef < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { self . instruction () . hash (state) ; } } impl PartialEq for RefSpec { fn eq (& self , other : & Self) -> bool { self . to_ref () . eq (& other . to_ref ()) } } impl PartialEq for RefSpecRef < '_ > { fn eq (& self , other : & Self) -> bool { self . instruction () . eq (& other . instruction ()) } } impl PartialOrd for RefSpecRef < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } } # [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd for RefSpec { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . to_ref () . cmp (& other . to_ref ())) } } impl Ord for RefSpecRef < '_ > { fn cmp (& self , other : & Self) -> Ordering { self . instruction () . cmp (& other . instruction ()) } } impl Ord for RefSpec { fn cmp (& self , other : & Self) -> Ordering { self . to_ref () . cmp (& other . to_ref ()) } } }
    };
}

impls!();