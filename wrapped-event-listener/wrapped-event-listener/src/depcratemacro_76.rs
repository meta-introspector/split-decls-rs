// Generated macro for macro_76 (macro)
macro_rules! Depcratemacro_76 {
() => {
// Module: crate
// Provides: {"macro_76"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [project (! Unpin)] # [project = ListenerProject] struct InnerListener < T , B : Borrow < Inner < T >>> where B : Unpin , { event : B , # [pin] listener : Option < sys :: Listener < T >>, } impl < T , B : Borrow < Inner < T >>> PinnedDrop for InnerListener < T , B > where B : Unpin , { fn drop (mut this : Pin <& mut Self >) { let this = this . project () ; (* this . event) . borrow () . remove (this . listener , true) ; } } }
};
}
