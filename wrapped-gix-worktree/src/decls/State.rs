macro_rules! deps {
    () => {
        Ignore!();
        Attributes!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [derive (Clone)] pub enum State { # [doc = " Useful for checkout where directories need creation, but we need to access attributes as well."] # [cfg (feature = "attributes")] CreateDirectoryAndAttributesStack { # [doc = " If there is a symlink or a file in our path, try to unlink it before creating the directory."] unlink_on_collision : bool , # [doc = " Options to control how newly created path components should be validated."] validate : gix_validate :: path :: component :: Options , # [doc = " State to handle attribute information"] attributes : state :: Attributes , } , # [doc = " Used when adding files, requiring access to both attributes and ignore information, for example during add operations."] # [cfg (feature = "attributes")] AttributesAndIgnoreStack { # [doc = " State to handle attribute information"] attributes : state :: Attributes , # [doc = " State to handle exclusion information"] ignore : state :: Ignore , } , # [doc = " Used when only attributes are required, typically with fully virtual worktrees."] # [cfg (feature = "attributes")] AttributesStack (state :: Attributes) , # [doc = " Used when providing worktree status information."] IgnoreStack (state :: Ignore) , }
    };
}

State!()