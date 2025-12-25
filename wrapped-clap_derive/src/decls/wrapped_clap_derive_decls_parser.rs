use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generates the `Parser` implementation.
///
/// This is far less verbose than defining the `clap::Command` struct manually,
/// receiving an instance of `clap::ArgMatches` from conducting parsing, and then
/// implementing a conversion code to instantiate an instance of the user
/// context struct.
#[proc_macro_derive(Parser, attributes(clap, structopt, command, arg, group))]
pub fn parser(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_parser(&input)
        .unwrap_or_else(|err| {
            let specific_dummy = match input.data {
                Data::Struct(DataStruct { fields: Fields::Named(ref _fields), .. }) => {
                    Some(dummies::args(&input.ident))
                }
                Data::Struct(DataStruct { fields: Fields::Unit, .. }) => {
                    Some(dummies::args(&input.ident))
                }
                Data::Enum(_) => Some(dummies::subcommand(&input.ident)),
                _ => None,
            };
            let dummy = specific_dummy
                .map(|specific_dummy| {
                    let parser_dummy = dummies::parser(&input.ident);
                    quote::quote! {
                        # parser_dummy # specific_dummy
                    }
                })
                .unwrap_or_else(|| quote::quote!());
            to_compile_error(err, dummy)
        })
        .into()
}
