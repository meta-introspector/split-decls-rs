macro_rules! Printer {
    () => {
        # [doc = " A printer for a regular expression's high-level intermediate"] # [doc = " representation."] # [doc = ""] # [doc = " A printer converts a high-level intermediate representation (HIR) to a"] # [doc = " regular expression pattern string. This particular printer uses constant"] # [doc = " stack space and heap space proportional to the size of the HIR."] # [doc = ""] # [doc = " Since this printer is only using the HIR, the pattern it prints will likely"] # [doc = " not resemble the original pattern at all. For example, a pattern like"] # [doc = " `\\pL` will have its entire class written out."] # [doc = ""] # [doc = " The purpose of this printer is to provide a means to mutate an HIR and then"] # [doc = " build a regular expression from the result of that mutation. (A regex"] # [doc = " library could provide a constructor from this HIR explicitly, but that"] # [doc = " creates an unnecessary public coupling between the regex library and this"] # [doc = " specific HIR representation.)"] # [derive (Debug)] pub struct Printer { _priv : () , }
    };
}

Printer!();