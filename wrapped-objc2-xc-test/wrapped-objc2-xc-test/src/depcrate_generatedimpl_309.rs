// Generated macro for impl_309 (impl)
macro_rules! Depcrate_generatedimpl_309 {
() => {
// Module: crate::generated
// Provides: {"impl_309"}
// Dependencies: {}
# [doc = " XCUIApplication_LaunchTesting."] impl XCTestCase { extern_methods ! (# [doc = " Determines whether the tests in this class should run multiple times, once for each of the target application's UI configurations."] # [doc = ""] # [doc = " Returns false by default. If overridden in a UI test subclass to return true, each test in that"] # [doc = " class will run multiple times, once for each supported UI configuration of the default target application."] # [doc = ""] # [doc = " Supported UI configurations are detected by Xcode according to the settings of the default target app"] # [doc = " for the UI test target and may include:"] # [doc = ""] # [doc = " - Appearances (e.g. light mode, dark mode)"] # [doc = " - Orientations (e.g. portrait, landscape)"] # [doc = " - Localizations (e.g. en_US, zh_CN)"] # [doc = ""] # [doc = " Given the above example, one UI configuration would be {dark mode, landscape, en_US}, another would be"] # [doc = " {light mode, portrait, zh_CN}, and so forth. The number of combinations determines the number of times each"] # [doc = " test will run. The UI configuration is used automatically when calling `XCUIApplication.launch()` in each test."] # [unsafe (method (runsForEachTargetApplicationUIConfiguration))] # [unsafe (method_family = none)] pub fn runsForEachTargetApplicationUIConfiguration () -> bool ;) ; }
};
}
