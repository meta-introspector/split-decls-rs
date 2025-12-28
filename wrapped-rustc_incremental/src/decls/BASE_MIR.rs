macro_rules! BASE_MIR {
    () => {
        # [doc = " DepNodes for exported mir bodies, which is relevant in \"executable\""] # [doc = " code, i.e., functions+methods"] const BASE_MIR : & [& str] = & [label_strs :: optimized_mir , label_strs :: promoted_mir] ;
    };
}

BASE_MIR!();