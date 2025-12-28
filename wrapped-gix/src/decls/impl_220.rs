macro_rules! deps {
    () => {
        Default!();
        Outcome!();
        Note!();
        Platform!();
        Action!();
        Tree!();
        Error!();
        Change!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        # [doc = " Add the item to compare to."] impl < 'old > Platform < '_ , 'old > { # [doc = " Call `for_each` repeatedly with all changes that are needed to convert the source of the diff to the tree to `other`."] # [doc = ""] # [doc = " `other` could also be created with the [`empty_tree()`][crate::Repository::empty_tree()] method to handle the first commit"] # [doc = " in a repository - it doesn't have a parent, equivalent to compare 'nothing' to something."] pub fn for_each_to_obtain_tree < 'new , E > (& mut self , other : & Tree < 'new > , for_each : impl FnMut (Change < '_ , 'old , 'new >) -> Result < Action , E > ,) -> Result < Option < gix_diff :: rewrites :: Outcome > , Error > where E : Into < Box < dyn std :: error :: Error + Sync + Send + 'static > > , { self . for_each_to_obtain_tree_inner (other , for_each , None) } # [doc = " Like [`Self::for_each_to_obtain_tree()`], but with a reusable `resource_cache` which is used to perform"] # [doc = " diffs fast."] # [doc = ""] # [doc = " Reusing it between multiple invocations saves a lot of IOps as it avoids the creation"] # [doc = " of a temporary `resource_cache` that triggers reading or checking for multiple gitattribute files."] # [doc = " Note that it's recommended to call [`gix_diff::blob::Platform::clear_resource_cache()`] between the calls"] # [doc = " to avoid runaway memory usage, as the cache isn't limited."] # [doc = ""] # [doc = " Note that to do rename tracking like `git` does, one has to configure the `resource_cache` with"] # [doc = " a conversion pipeline that uses [`gix_diff::blob::pipeline::Mode::ToGit`]."] pub fn for_each_to_obtain_tree_with_cache < 'new , E > (& mut self , other : & Tree < 'new > , resource_cache : & mut gix_diff :: blob :: Platform , for_each : impl FnMut (Change < '_ , 'old , 'new >) -> Result < Action , E > ,) -> Result < Option < gix_diff :: rewrites :: Outcome > , Error > where E : Into < Box < dyn std :: error :: Error + Sync + Send + 'static > > , { self . for_each_to_obtain_tree_inner (other , for_each , Some (resource_cache)) } fn for_each_to_obtain_tree_inner < 'new , E > (& mut self , other : & Tree < 'new > , mut for_each : impl FnMut (Change < '_ , 'old , 'new >) -> Result < Action , E > , resource_cache : Option < & mut gix_diff :: blob :: Platform > ,) -> Result < Option < gix_diff :: rewrites :: Outcome > , Error > where E : Into < Box < dyn std :: error :: Error + Sync + Send + 'static > > , { let repo = self . lhs . repo ; let mut storage ; let cache = match resource_cache { None => { storage = repo . diff_resource_cache (gix_diff :: blob :: pipeline :: Mode :: ToGit , Default :: default ()) ? ; & mut storage } Some (cache) => cache , } ; let opts = self . options . into () ; Ok (gix_diff :: tree_with_rewrites (TreeRefIter :: from_bytes (& self . lhs . data) , TreeRefIter :: from_bytes (& other . data) , cache , & mut self . state , & repo . objects , | change | { for_each (Change :: from_change_ref (change , repo , other . repo)) . map (| action | match action { Action :: Continue => gix_diff :: tree_with_rewrites :: Action :: Continue , Action :: Cancel => gix_diff :: tree_with_rewrites :: Action :: Cancel , }) } , opts ,) ?) } }
    };
}

impl_220!()