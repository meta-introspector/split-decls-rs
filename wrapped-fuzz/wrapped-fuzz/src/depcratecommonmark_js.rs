// Generated macro for commonmark_js (function)
macro_rules! Depcratecommonmark_js {
() => {
// Module: crate
// Provides: {"commonmark_js"}
// Dependencies: {}
# [doc = " Send Markdown `text` to `commonmark.js` and return XML."] pub fn commonmark_js (text : & str) -> anyhow :: Result < String > { const COMMONMARK_MIN_JS : & str = include_str ! ("../../pulldown-cmark/third_party/commonmark.js/commonmark.min.js") ; thread_local ! { static ENGINE : JSEngine = { JSEngine :: init () . expect ("failed to initalize JS engine") } } ENGINE . with (| engine | { let rt = Runtime :: new (engine . handle ()) ; let options = RealmOptions :: default () ; rooted ! (in (rt . cx ()) let global = unsafe { JS_NewGlobalObject (rt . cx () , & SIMPLE_GLOBAL_CLASS , ptr :: null_mut () , OnNewGlobalHookOption :: FireOnNewGlobalHook , &* options) }) ; let realm = unsafe { EnterRealm (rt . cx () , global . get ()) } ; rooted ! (in (rt . cx ()) let mut rval = UndefinedValue ()) ; let filename : & 'static str = "commonmark.min.js" ; let lineno : u32 = 1 ; let res = rt . evaluate_script (global . handle () , COMMONMARK_MIN_JS , filename , lineno , rval . handle_mut () ,) ; assert ! (res . is_ok ()) ; let filename : & 'static str = "{inline}" ; let lineno : u32 = 1 ; let script = r#"
            function render_to_xml(markdown) {
                var reader = new commonmark.Parser();
                var xmlwriter = new commonmark.XmlRenderer({ sourcepos: false });
                return xmlwriter.render(reader.parse(markdown));
            }
        "# ; rooted ! (in (rt . cx ()) let mut render_to_xml = UndefinedValue ()) ; let res = rt . evaluate_script (global . handle () , script , filename , lineno , render_to_xml . handle_mut () ,) ; assert ! (res . is_ok ()) ; let xml = unsafe { rooted ! (in (rt . cx ()) let mut xml = UndefinedValue ()) ; rooted ! (in (rt . cx ()) let mut text_val = UndefinedValue ()) ; text . to_jsval (rt . cx () , text_val . handle_mut ()) ; JS_CallFunctionName (rt . cx () , global . handle () , b"render_to_xml\0" . as_ptr () as * const i8 , & HandleValueArray :: from_rooted_slice (& [text_val . handle () . get ()]) , xml . handle_mut () ,) ; let xml_string = xml . handle () . to_string () ; let utf8 = mozjs :: conversions :: jsstr_to_string (rt . cx () , xml_string) ; utf8 } ; unsafe { LeaveRealm (rt . cx () , realm) ; } Ok (xml) }) }
};
}
