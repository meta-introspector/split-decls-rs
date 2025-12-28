macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! Limb {
    () => {
        deps!();
        # [doc = " Big integers are represented as an array/vector of smaller CPU word-size integers called"] # [doc = " \"limbs\"."] # [doc = ""] # [doc = " The [`Limb`] type uses a 32-bit or 64-bit saturated representation, depending on the target."] # [doc = " All bits of an inner [`Word`] are used to represent larger big integer types."] # [allow (clippy :: derived_hash_with_manual_eq)] # [derive (Copy , Clone , Default , Hash)] # [repr (transparent)] pub struct Limb (pub Word) ;
    };
}

Limb!()