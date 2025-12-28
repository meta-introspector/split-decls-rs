macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! CfgOptions {
    () => {
        deps!();
        # [doc = " Configuration options used for conditional compilation on items with `cfg` attributes."] # [doc = " We have two kind of options in different namespaces: atomic options like `unix`, and"] # [doc = " key-value options like `target_arch=\"x86\"`."] # [doc = ""] # [doc = " Note that for key-value options, one key can have multiple values (but not none)."] # [doc = " `feature` is an example. We have both `feature=\"foo\"` and `feature=\"bar\"` if features"] # [doc = " `foo` and `bar` are both enabled. And here, we store key-value options as a set of tuple"] # [doc = " of key and value in `key_values`."] # [doc = ""] # [doc = " See: <https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options>"] # [derive (Clone , PartialEq , Eq)] pub struct CfgOptions { enabled : FxHashSet < CfgAtom > , }
    };
}

CfgOptions!();