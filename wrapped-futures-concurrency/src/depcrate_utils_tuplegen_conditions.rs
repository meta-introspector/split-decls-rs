// Generated macro for gen_conditions (macro)
macro_rules! Depcrate_utils_tuplegen_conditions {
() => {
// Module: crate::utils::tuple
// Provides: {"gen_conditions"}
// Dependencies: {}
# [doc = " Generate the `match` conditions inside the main polling body. This macro"] # [doc = " chooses a random starting point on each call to the given method, making"] # [doc = " it \"fair\"."] # [doc = ""] # [doc = " The way this algorithm works is: we generate a random number between 0 and"] # [doc = " the length of the tuple we have. This number determines which element we"] # [doc = " start with. All other cases are mapped as `r + index`, and after we have the"] # [doc = " first one, we'll sequentially iterate over all others. The starting point of"] # [doc = " the stream is random, but the iteration order of all others is not."] macro_rules ! gen_conditions { ($ i : expr , $ this : expr , $ cx : expr , $ method : ident , $ (($ F_index : expr ; $ F : ident , { $ ($ arms : pat => $ foo : expr ,) * })) *) => { $ (if $ i == $ F_index { match unsafe { Pin :: new_unchecked (& mut $ this .$ F) } .$ method ($ cx) { $ ($ arms => $ foo ,) * } }) * } }
};
}
