// Generated macro for relative_traversal (function)
macro_rules! Depcrate_relative_path_bufrelative_traversal {
() => {
// Module: crate::relative_path_buf
// Provides: {"relative_traversal"}
// Dependencies: {}
# [doc = " Traverse the given components and apply to the provided stack."] # [doc = ""] # [doc = " This takes '.', and '..' into account. Where '.' doesn't change the stack, and '..' pops the"] # [doc = " last item or further adds parent components."] # [inline (always)] pub (super) fn relative_traversal < 'a , C > (buf : & mut RelativePathBuf , components : C) where C : IntoIterator < Item = Component < 'a > > , { use self :: Component :: { CurDir , Normal , ParentDir } ; for c in components { match c { CurDir => () , ParentDir => match buf . components () . next_back () { Some (Component :: ParentDir) | None => { buf . push (PARENT_STR) ; } _ => { buf . pop () ; } } , Normal (name) => { buf . push (name) ; } } } }
};
}
