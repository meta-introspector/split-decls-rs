macro_rules! PrefixEntryPoint {
    () => {
        # [doc = " Parse a prefix of the input as a given syntactic construct."] # [doc = ""] # [doc = " This is used by macro-by-example parser to implement things like `$i:item`"] # [doc = " and the naming of variants follows the naming of macro fragments."] # [doc = ""] # [doc = " Note that this is generally non-optional -- the result is intentionally not"] # [doc = " `Option<Output>`. The way MBE work, by the time we *try* to parse `$e:expr`"] # [doc = " we already commit to expression. In other words, this API by design can't be"] # [doc = " used to implement \"rollback and try another alternative\" logic."] # [derive (Debug)] pub enum PrefixEntryPoint { Vis , Block , Stmt , Pat , PatTop , Ty , Expr , Path , Item , MetaItem , }
    };
}

PrefixEntryPoint!()