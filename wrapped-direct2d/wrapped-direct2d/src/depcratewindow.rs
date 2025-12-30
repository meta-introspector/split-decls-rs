// Generated macro for Window (struct)
macro_rules! DepcrateWindow {
() => {
// Module: crate
// Provides: {"Window"}
// Dependencies: {}
struct Window { handle : HWND , factory : ID2D1Factory1 , dxfactory : IDXGIFactory2 , style : ID2D1StrokeStyle1 , manager : IUIAnimationManager , variable : IUIAnimationVariable , target : Option < ID2D1DeviceContext > , swapchain : Option < IDXGISwapChain1 > , brush : Option < ID2D1SolidColorBrush > , shadow : Option < ID2D1Effect > , clock : Option < ID2D1Bitmap1 > , dpi : f32 , visible : bool , occlusion : u32 , frequency : i64 , angles : Angles , }
};
}
