// Generated macro for create_text_format (function)
macro_rules! Depcratecreate_text_format {
() => {
// Module: crate
// Provides: {"create_text_format"}
// Dependencies: {}
fn create_text_format () -> Result < IDWriteTextFormat > { unsafe { let factory : IDWriteFactory2 = DWriteCreateFactory (DWRITE_FACTORY_TYPE_SHARED) ? ; let format = factory . CreateTextFormat (w ! ("Candara") , None , DWRITE_FONT_WEIGHT_NORMAL , DWRITE_FONT_STYLE_NORMAL , DWRITE_FONT_STRETCH_NORMAL , CARD_HEIGHT / 2.0 , w ! ("en") ,) ? ; format . SetTextAlignment (DWRITE_TEXT_ALIGNMENT_CENTER) ? ; format . SetParagraphAlignment (DWRITE_PARAGRAPH_ALIGNMENT_CENTER) ? ; Ok (format) } }
};
}
