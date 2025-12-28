macro_rules! deps {
    () => {
        Data!();
        Features!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [doc = " Syntax tree type defined by Syn."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Node { # [doc = " Name of the type."] # [doc = ""] # [doc = " This type is accessible in the Syn public API as `syn::#name`."] pub ident : String , # [doc = " Features behind which this type is cfg gated."] pub features : Features , # [doc = " Content of the data structure."] # [cfg_attr (feature = "serde" , serde (flatten , skip_serializing_if = "is_private" , deserialize_with = "private_if_absent"))] pub data : Data , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "is_true" , default = "bool_true"))] pub exhaustive : bool , }
    };
}

Node!();