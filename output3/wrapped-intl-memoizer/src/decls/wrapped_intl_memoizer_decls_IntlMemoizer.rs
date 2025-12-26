use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// [`IntlMemoizer`] is designed to handle lazily-initialized references to
/// internationalization formatters.
///
/// Constructing a new formatter is often expensive in terms of memory and performance,
/// and the instance is often read-only during its lifetime. The format operations in
/// comparison are relatively cheap.
///
/// Because of this relationship, it can be helpful to memoize the constructors, and
/// re-use them across multiple format operations. This strategy is used where all
/// instances of intl APIs such as `PluralRules`, `DateTimeFormat` etc. are memoized
/// between all `FluentBundle` instances.
///
/// # Example
///
/// For a more complete example of the memoization, see the [`IntlLangMemoizer`] documentation.
/// This example provides a higher-level overview.
///
/// ```
/// # use intl_memoizer::{IntlMemoizer, IntlLangMemoizer, Memoizable};
/// # use unic_langid::LanguageIdentifier;
/// # use std::rc::Rc;
/// #
/// # struct ExampleFormatter {
/// #     lang: LanguageIdentifier,
/// #     prefix: String,
/// # }
/// #
/// # impl ExampleFormatter {
/// #     fn format(&self, example_string: &str) -> String {
/// #         format!(
/// #             "{} lang({}) string({})",
/// #             self.prefix, self.lang, example_string
/// #         )
/// #     }
/// # }
/// #
/// # impl Memoizable for ExampleFormatter {
/// #     type Args = (String,);
/// #     type Error = ();
/// #     fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
/// #         Ok(Self {
/// #             lang,
/// #             prefix: args.0,
/// #         })
/// #     }
/// # }
/// #
/// let mut memoizer = IntlMemoizer::default();
///
/// // The memoziation happens per-locale.
/// let en_us = "en-US".parse().expect("Failed to parse.");
/// let en_us_memoizer: Rc<IntlLangMemoizer> = memoizer.get_for_lang(en_us);
///
/// // These arguments are passed into the constructor for `ExampleFormatter`. The
/// // construct_args will be used for determining the memoization, but the message
/// // can be different and re-use the constructed instance.
/// let construct_args = (String::from("prefix:"),);
/// let message = "The format operation will run";
///
/// // Use the `ExampleFormatter` from the `IntlLangMemoizer` example. It returns a
/// // string that demonstrates the configuration of the formatter. This step will
/// // construct a new formatter if needed, and run the format operation.
/// //
/// // See `IntlLangMemoizer` for more details on this step.
/// let en_us_result = en_us_memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message)
///     });
///
/// // The example formatter constructs a string with diagnostic information about
/// // the configuration.
/// assert_eq!(
///     en_us_result.unwrap(),
///     "prefix: lang(en-US) string(The format operation will run)"
/// );
///
/// // The process can be repeated for a new locale.
///
/// let de_de = "de-DE".parse().expect("Failed to parse.");
/// let de_de_memoizer: Rc<IntlLangMemoizer> = memoizer.get_for_lang(de_de);
///
/// let de_de_result = de_de_memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message)
///     });
///
/// assert_eq!(
///     de_de_result.unwrap(),
///     "prefix: lang(de-DE) string(The format operation will run)"
/// );
/// ```
#[derive(Default)]
pub struct IntlMemoizer {
    map: HashMap<LanguageIdentifier, Weak<IntlLangMemoizer>>,
}
