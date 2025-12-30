// Generated macro for impl_657 (impl)
macro_rules! Depcrate_output_render_blocksimpl_657 {
() => {
// Module: crate::output::render::blocks
// Provides: {"impl_657"}
// Dependencies: {}
impl f :: Blocksize { pub fn render < C : Colours > (self , colours : & C , size_format : SizeFormat , numerics : & NumericLocale ,) -> TextCell { use number_prefix :: NumberPrefix ; let size = match self { Self :: Some (s) => s , Self :: None => return TextCell :: blank (colours . no_blocksize ()) , } ; let result = match size_format { SizeFormat :: DecimalBytes => NumberPrefix :: decimal (size as f64) , SizeFormat :: BinaryBytes => NumberPrefix :: binary (size as f64) , SizeFormat :: JustBytes => { let prefix = match NumberPrefix :: binary (size as f64) { NumberPrefix :: Standalone (_) => None , NumberPrefix :: Prefixed (p , _) => Some (p) , } ; let string = numerics . format_int (size) ; return TextCell :: paint (colours . blocksize (prefix) , string) ; } } ; let (prefix , n) = match result { NumberPrefix :: Standalone (b) => { return TextCell :: paint (colours . blocksize (None) , numerics . format_int (b)) } NumberPrefix :: Prefixed (p , n) => (p , n) , } ; let symbol = prefix . symbol () ; let number = if n < 10_f64 { numerics . format_float (n , 1) } else { numerics . format_int (n . round () as isize) } ; TextCell { width : DisplayWidth :: from (& * number) + symbol . len () , contents : vec ! [colours . blocksize (Some (prefix)) . paint (number) , colours . unit (Some (prefix)) . paint (symbol) ,] . into () , } } }
};
}
