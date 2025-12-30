// Generated macro for impl_67 (impl)
macro_rules! Depcrate_render_tui_draw_progressimpl_67 {
() => {
// Module: crate::render::tui::draw::progress
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Display for ProgressFormat < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 { Some (p) => match p . unit . as_ref () { Some (unit) => write ! (f , "{}" , unit . display (p . step . load (Ordering :: SeqCst) , p . done_at , self . 2 . clone ())) , None => match p . done_at { Some (done_at) => write ! (f , "{}/{}" , p . step . load (Ordering :: SeqCst) , done_at) , None => write ! (f , "{}" , p . step . load (Ordering :: SeqCst)) , } , } , None => write ! (f , "{:─<width$}" , '─' , width = self . 1 as usize) , } } }
};
}
