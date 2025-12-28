macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! Definitions {
    () => {
        deps!();
        # [doc = " Top-level content of the syntax tree description."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Definitions { # [doc = " The Syn version whose syntax tree is described by this data."] pub version : Version , # [doc = " Syntax tree types defined by Syn."] pub types : Vec < Node > , # [doc = " Token types defined by Syn (keywords as well as punctuation)."] # [doc = ""] # [doc = " The keys in the map are the Rust type name for the token. The values in"] # [doc = " the map are the printed token representation."] # [doc = ""] # [doc = " These tokens are accessible in the Syn public API as `syn::token::#name`"] # [doc = " or alternatively `syn::Token![#repr]`."] pub tokens : BTreeMap < String , String > , }
    };
}

Definitions!()