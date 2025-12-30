// Generated macro for DnsQuestion (struct)
macro_rules! Depcrate_dns_questionDnsQuestion {
() => {
// Module: crate::dns_question
// Provides: {"DnsQuestion"}
// Dependencies: {}
# [doc = " > The question section is used to carry the \"question\" in most queries, i.e., the parameters"] # [doc = " > that define what is being asked.  The section contains QDCOUNT (usually 1) entries, each of"] # [doc = " > the following format:"] # [doc = " >"] # [doc = " > ```text"] # [doc = " >                                 1  1  1  1  1  1"] # [doc = " >   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5"] # [doc = " > +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+"] # [doc = " > |                                               |"] # [doc = " > /                     QNAME                     /"] # [doc = " > /                                               /"] # [doc = " > +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+"] # [doc = " > |                     QTYPE                     |"] # [doc = " > +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+"] # [doc = " > |                     QCLASS                    |"] # [doc = " > +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+"] # [doc = " > ```"] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct DnsQuestion { pub name : DnsName , pub typ : DnsType , pub class : DnsClass , }
};
}
