// Generated macro for ElementEnum (enum)
macro_rules! Depcrate_rcdomElementEnum {
() => {
// Module: crate::rcdom
// Provides: {"ElementEnum"}
// Dependencies: {}
# [doc = " The different kinds of elements in the DOM."] # [derive (Debug)] pub enum ElementEnum { Normal , # [doc = " A script element and its \"already started\" flag."] # [doc = " https://html.spec.whatwg.org/multipage/#already-started"] Script (bool) , # [doc = " A template element and its template contents."] # [doc = " https://html.spec.whatwg.org/multipage/#template-contents"] Template (Handle) , # [doc = " An annotation-xml element in the MathML namespace whose start tag token had an attribute"] # [doc = " with the name \"encoding\" whose value was an ASCII case-insensitive match for the string"] # [doc = " \"text/html\" or \"application/xhtml+xml\""] # [doc = " https://html.spec.whatwg.org/multipage/embedded-content.html#math:annotation-xml"] AnnotationXml (bool) , }
};
}
