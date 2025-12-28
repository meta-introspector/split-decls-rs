macro_rules! CommentDef {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (untagged))] pub enum CommentDef < S > { Single { content : S } , Multi { content : Vec < S > } , }
    };
}

CommentDef!()