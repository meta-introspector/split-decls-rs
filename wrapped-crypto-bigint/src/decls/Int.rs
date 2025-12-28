macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! Int {
    () => {
        deps!();
        # [doc = " Stack-allocated big _signed_ integer."] # [doc = " See [`Uint`] for _unsigned_ integers."] # [doc = ""] # [doc = " Created as a [`Uint`] newtype."] # [allow (clippy :: derived_hash_with_manual_eq)] # [derive (Copy , Clone , Hash)] # [repr (transparent)] pub struct Int < const LIMBS : usize > (Uint < LIMBS >) ;
    };
}

Int!();