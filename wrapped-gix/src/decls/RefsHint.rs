macro_rules! deps {
    () => {
        Note!();
        Clone!();
        Default!();
    };
}

macro_rules! RefsHint {
    () => {
        deps!();
        # [doc = " A hint to know what to do if refs and object names are equal."] # [derive (Default , Debug , Copy , Clone , PartialEq , Eq)] pub enum RefsHint { # [doc = " This is the default, and leads to specs that look like objects identified by full hex sha and are objects to be used"] # [doc = " instead of similarly named references. The latter is not typical but can absolutely happen by accident."] # [doc = " If the object prefix is shorter than the maximum hash length of the repository, use the reference instead, which is"] # [doc = " preferred as there are many valid object names like `beef` and `cafe` that are short and both valid and typical prefixes"] # [doc = " for objects."] # [doc = " Git chooses this as default as well, even though it means that every object prefix is also looked up as ref."] # [default] PreferObjectOnFullLengthHexShaUseRefOtherwise , # [doc = " No matter what, if it looks like an object prefix and has an object, use it."] # [doc = " Note that no ref-lookup is made here which is the fastest option."] PreferObject , # [doc = " When an object is found for a given prefix, also check if a reference exists with that name and if it does,"] # [doc = " use that moving forward."] PreferRef , # [doc = " If there is an ambiguous situation, instead of silently choosing one over the other, fail instead."] Fail , }
    };
}

RefsHint!()