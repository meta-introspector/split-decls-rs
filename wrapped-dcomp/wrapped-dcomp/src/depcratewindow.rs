// Generated macro for Window (struct)
macro_rules! DepcrateWindow {
() => {
// Module: crate
// Provides: {"Window"}
// Dependencies: {}
struct Window { handle : HWND , dpi : (f32 , f32) , format : IDWriteTextFormat , image : IWICFormatConverter , manager : IUIAnimationManager2 , library : IUIAnimationTransitionLibrary2 , first : Option < usize > , cards : Vec < Card > , device : Option < ID3D11Device > , desktop : Option < IDCompositionDesktopDevice > , target : Option < IDCompositionTarget > , }
};
}
