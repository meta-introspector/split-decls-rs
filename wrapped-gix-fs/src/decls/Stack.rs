macro_rules! Stack {
    () => {
        # [doc = " A stack of path components with the delegation of side-effects as the currently set path changes, component by component."] # [derive (Clone)] pub struct Stack { # [doc = " The prefix/root for all paths we handle."] root : PathBuf , # [doc = " the most recent known cached that we know is valid."] current : PathBuf , # [doc = " The relative portion of `valid` that was added previously."] current_relative : PathBuf , # [doc = " The amount of path components of 'current' beyond the roots components."] valid_components : usize , # [doc = " If set, we assume the `current` element is a directory to affect calls to `(push|pop)_directory()`."] current_is_directory : bool , }
    };
}

Stack!()