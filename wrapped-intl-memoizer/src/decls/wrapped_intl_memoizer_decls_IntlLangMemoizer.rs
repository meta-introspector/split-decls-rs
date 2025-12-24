use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [`IntlLangMemoizer`] can memoize multiple constructed internationalization
/// formatters, and their configuration for a single locale. For instance, given "en-US",
/// a memorizer could retain 3 `DateTimeFormat` instances, and a `PluralRules`.
///
/// For memoizing with multiple locales, see [`IntlMemoizer`].
///
/// # Example
///
/// The code example does the following steps:
///
/// 1. Create a static counter
/// 2. Create an `ExampleFormatter`
/// 3. Implement [`Memoizable`] for `ExampleFormatter`.
/// 4. Use `IntlLangMemoizer::with_try_get` to run `ExampleFormatter::format`
/// 5. Demonstrate the memoization using the static counter
///
/// ```
/// use intl_memoizer::{IntlLangMemoizer, Memoizable};
/// use unic_langid::LanguageIdentifier;
///
/// // Create a static counter so that we can demonstrate the side effects of when
/// // the memoizer re-constructs an API.
///
/// static mut INTL_EXAMPLE_CONSTRUCTS: u32 = 0;
/// fn increment_constructs() {
///     unsafe {
///         INTL_EXAMPLE_CONSTRUCTS += 1;
///     }
/// }
///
/// fn get_constructs_count() -> u32 {
///     unsafe { INTL_EXAMPLE_CONSTRUCTS }
/// }
///
/// /// Create an example formatter, that doesn't really do anything useful. In a real
/// /// implementation, this could be a PluralRules or DateTimeFormat struct.
/// struct ExampleFormatter {
///     lang: LanguageIdentifier,
///     /// This is here to show how to initiate the API with an argument.
///     prefix: String,
/// }
///
/// impl ExampleFormatter {
///     /// Perform an example format by printing information about the formatter
///     /// configuration, and the arguments passed into the individual format operation.
///     fn format(&self, example_string: &str) -> String {
///         format!(
///             "{} lang({}) string({})",
///             self.prefix, self.lang, example_string
///         )
///     }
/// }
///
/// /// Multiple classes of structs may be add1ed to the memoizer, with the restriction
/// /// that they must implement the `Memoizable` trait.
/// impl Memoizable for ExampleFormatter {
///     /// The arguments will be passed into the constructor. Here a single `String`
///     /// will be used as a prefix to the formatting operation.
///     type Args = (String,);
///
///     /// If the constructor is fallible, than errors can be described here.
///     type Error = ();
///
///     /// This function wires together the `Args` and `Error` type to construct
///     /// the intl API. In our example, there is
///     fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
///         // Keep track for example purposes that this was constructed.
///         increment_constructs();
///
///         Ok(Self {
///             lang,
///             prefix: args.0,
///         })
///     }
/// }
///
/// // The following demonstrates how these structs are actually used with the memoizer.
///
/// // Construct a new memoizer.
/// let lang = "en-US".parse().expect("Failed to parse.");
/// let memoizer = IntlLangMemoizer::new(lang);
///
/// // These arguments are passed into the constructor for `ExampleFormatter`.
/// let construct_args = (String::from("prefix:"),);
/// let message1 = "The format operation will run";
/// let message2 = "ExampleFormatter will be re-used, when a second format is run";
///
/// // Run `IntlLangMemoizer::with_try_get`. The name of the method means "with" an
/// // intl formatter, "try and get" the result. See the method documentation for
/// // more details.
///
/// let result1 = memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message1)
///     });
///
/// // The memoized instance of `ExampleFormatter` will be re-used.
/// let result2 = memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message2)
///     });
///
/// assert_eq!(
///     result1.unwrap(),
///     "prefix: lang(en-US) string(The format operation will run)"
/// );
/// assert_eq!(
///     result2.unwrap(),
///     "prefix: lang(en-US) string(ExampleFormatter will be re-used, when a second format is run)"
/// );
/// assert_eq!(
///     get_constructs_count(),
///     1,
///     "The constructor was only run once."
/// );
///
/// let construct_args = (String::from("re-init:"),);
///
/// // Since the constructor args changed, `ExampleFormatter` will be re-constructed.
/// let result1 = memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message1)
///     });
///
/// // The memoized instance of `ExampleFormatter` will be re-used.
/// let result2 = memoizer
///     .with_try_get::<ExampleFormatter, _, _>(construct_args.clone(), |intl_example| {
///         intl_example.format(message2)
///     });
///
/// assert_eq!(
///     result1.unwrap(),
///     "re-init: lang(en-US) string(The format operation will run)"
/// );
/// assert_eq!(
///     result2.unwrap(),
///     "re-init: lang(en-US) string(ExampleFormatter will be re-used, when a second format is run)"
/// );
/// assert_eq!(
///     get_constructs_count(),
///     2,
///     "The constructor was invalidated and ran again."
/// );
/// ```
#[derive(Debug)]
pub struct IntlLangMemoizer {
    lang: LanguageIdentifier,
    map: RefCell<type_map::TypeMap>,
}
