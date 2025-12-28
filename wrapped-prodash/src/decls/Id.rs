macro_rules! Id {
    () => {
        # [doc = " Four bytes of function-local unique and stable identifier for each item added as progress,"] # [doc = " like b\"TREE\" or b\"FILE\"."] # [doc = ""] # [doc = " Note that uniqueness only relates to one particular method call where those interested in its progress"] # [doc = " may assume certain stable ids to look for when selecting specific bits of progress to process."] pub type Id = [u8 ; 4] ;
    };
}

Id!();