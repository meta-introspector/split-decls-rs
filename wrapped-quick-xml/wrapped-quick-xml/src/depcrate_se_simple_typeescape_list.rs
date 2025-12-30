// Generated macro for escape_list (function)
macro_rules! Depcrate_se_simple_typeescape_list {
() => {
// Module: crate::se::simple_type
// Provides: {"escape_list"}
// Dependencies: {}
# [doc = " Escapes XSD simple type value"] fn escape_list < W > (mut writer : W , value : & str , target : QuoteTarget , level : QuoteLevel) -> fmt :: Result where W : Write , { use QuoteLevel :: * ; use QuoteTarget :: * ; match (target , level) { (CData , _) => { for part in CDataIterator :: new (value) { writer . write_str ("<![CDATA[") ? ; writer . write_str (part) ? ; writer . write_str ("]]>") ? ; } Ok (()) } (_ , Full) => escape_into (writer , value , | ch | match ch { b'&' | b'<' | b'>' | b'\'' | b'\"' => true , _ => false , }) , (Text , Partial) => escape_into (writer , value , | ch | match ch { b'&' | b'<' | b'>' => true , _ => false , }) , (Text , Minimal) => escape_into (writer , value , | ch | match ch { b'&' | b'<' => true , _ => false , }) , (DoubleQAttr , Partial) => escape_into (writer , value , | ch | match ch { b'&' | b'<' | b'>' => true , b'"' => true , _ => false , }) , (DoubleQAttr , Minimal) => escape_into (writer , value , | ch | match ch { b'&' | b'<' => true , b'"' => true , _ => false , }) , (SingleQAttr , Partial) => escape_into (writer , value , | ch | match ch { b'&' | b'<' | b'>' => true , b'\'' => true , _ => false , }) , (SingleQAttr , Minimal) => escape_into (writer , value , | ch | match ch { b'&' | b'<' => true , b'\'' => true , _ => false , }) , } }
};
}
