// Generated macro for notify_callbacks (function)
macro_rules! Depcrate_utilsnotify_callbacks {
() => {
// Module: crate::utils
// Provides: {"notify_callbacks"}
// Dependencies: {}
pub (crate) fn notify_callbacks (callbacks : Rc < RefCell < Vec < WeakCallback > > >) { let callables = { let mut callbacks_ref = callbacks . borrow_mut () ; let (callbacks , callbacks_weak) = callbacks_ref . iter () . cloned () . fold ((Vec :: new () , Vec :: new ()) , | (mut callbacks , mut callbacks_weak) , m | { if let Some (m_strong) = m . clone () . upgrade () { callbacks . push (m_strong) ; callbacks_weak . push (m) ; } (callbacks , callbacks_weak) } ,) ; * callbacks_ref = callbacks_weak ; callbacks } ; for callback in callables { callback () } }
};
}
