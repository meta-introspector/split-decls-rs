macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! StatusShow {
    () => {
        deps!();
        # [doc = " Enumeration of possible methods of what can be shown through a status"] # [doc = " operation."] # [derive (Copy , Clone)] pub enum StatusShow { # [doc = " Only gives status based on HEAD to index comparison, not looking at"] # [doc = " working directory changes."] Index , # [doc = " Only gives status based on index to working directory comparison, not"] # [doc = " comparing the index to the HEAD."] Workdir , # [doc = " The default, this roughly matches `git status --porcelain` regarding"] # [doc = " which files are included and in what order."] IndexAndWorkdir , }
    };
}

StatusShow!();