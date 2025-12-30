// Generated macro for DnsError (enum)
macro_rules! DepcrateDnsError {
() => {
// Module: crate
// Provides: {"DnsError"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , Hash , PartialEq)] pub enum DnsError { InvalidClass , InvalidLabel , InvalidOpCode , NameTooLong , NoQuestion , NotARequest , NotFound , RecordHasAdditionalBytes , ResponseBufferFull , QueryHasAdditionalRecords , QueryHasAnswer , QueryHasNameServer , StringTooLong , TooManyAdditional , TooManyAnswers , TooManyLabels , TooManyNameServers , TooManyQuestions , Truncated , Internal (String) , Unreachable (& 'static str , u32) , }
};
}
