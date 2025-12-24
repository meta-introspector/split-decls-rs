use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "pii", vis = "pub", hash = "7f2215df")]
pub fn pii(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let data = input_str.value();
    quote! {
        { let cleaned = # data.replace("/home/", "/home/<user>/").replace("/Users/",
        "/Users/<user>/").replace("@gmail.com", "@<email>").replace("@company.com",
        "@<company>"); println!("cargo:warning=🔒 PII cleaned"); cleaned }
    }
        .into()
}
