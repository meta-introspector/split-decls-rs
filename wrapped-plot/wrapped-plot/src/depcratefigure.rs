// Generated macro for Figure (struct)
macro_rules! DepcrateFigure {
() => {
// Module: crate
// Provides: {"Figure"}
// Dependencies: {}
# [doc = " Plot container"] # [derive (Clone)] pub struct Figure { alpha : Option < f64 > , axes : map :: axis :: Map < axis :: Properties > , box_width : Option < f64 > , font : Option < Cow < 'static , str > > , font_size : Option < f64 > , key : Option < key :: Properties > , output : Cow < 'static , Path > , plots : Vec < Plot > , size : Option < (usize , usize) > , terminal : Terminal , tics : map :: axis :: Map < String > , title : Option < Cow < 'static , str > > , }
};
}
