macro_rules! CommitTimeOrder {
    () => {
        # [derive (Default , Debug , Copy , Clone)] # [doc = " The order with which to prioritize the search."] pub enum CommitTimeOrder { # [default] # [doc = " Sort commits by newest first."] NewestFirst , # [doc = " Sort commits by oldest first."] # [doc (alias = "Sort::REVERSE" , alias = "git2")] OldestFirst , }
    };
}

CommitTimeOrder!()