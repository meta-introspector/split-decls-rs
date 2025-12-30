// Generated macro for EncodingRef (enum)
macro_rules! Depcrate_readerEncodingRef {
() => {
// Module: crate::reader
// Provides: {"EncodingRef"}
// Dependencies: {}
# [doc = " A reference to an encoding together with information about how it was retrieved."] # [doc = ""] # [doc = " The state transition diagram:"] # [doc = ""] # [doc = " ```mermaid"] # [doc = " flowchart LR"] # [doc = "   Implicit    -- from_str       --> Explicit"] # [doc = "   Implicit    -- BOM            --> BomDetected"] # [doc = "   Implicit    -- \"encoding=...\" --> XmlDetected"] # [doc = "   BomDetected -- \"encoding=...\" --> XmlDetected"] # [doc = " ```"] # [cfg (feature = "encoding")] # [derive (Clone , Copy , Debug)] enum EncodingRef { # [doc = " Encoding was implicitly assumed to have a specified value. It can be refined"] # [doc = " using BOM or by the XML declaration event (`<?xml encoding=... ?>`)"] Implicit (& 'static Encoding) , # [doc = " Encoding was explicitly set to the desired value. It cannot be changed"] # [doc = " nor by BOM, nor by parsing XML declaration (`<?xml encoding=... ?>`)"] Explicit (& 'static Encoding) , # [doc = " Encoding was detected from a byte order mark (BOM) or by the first bytes"] # [doc = " of the content. It can be refined by the XML declaration event (`<?xml encoding=... ?>`)"] BomDetected (& 'static Encoding) , # [doc = " Encoding was detected using XML declaration event (`<?xml encoding=... ?>`)."] # [doc = " It can no longer change"] XmlDetected (& 'static Encoding) , }
};
}
