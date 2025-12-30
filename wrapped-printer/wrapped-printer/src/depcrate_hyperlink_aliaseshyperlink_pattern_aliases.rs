// Generated macro for HYPERLINK_PATTERN_ALIASES (const)
macro_rules! Depcrate_hyperlink_aliasesHYPERLINK_PATTERN_ALIASES {
() => {
// Module: crate::hyperlink::aliases
// Provides: {"HYPERLINK_PATTERN_ALIASES"}
// Dependencies: {}
# [doc = " Aliases to well-known hyperlink schemes."] # [doc = ""] # [doc = " These need to be sorted by name."] pub (super) const HYPERLINK_PATTERN_ALIASES : & [HyperlinkAlias] = & [alias ("cursor" , "Cursor scheme (cursor://)" , "cursor://file{path}:{line}:{column}" ,) , prioritized_alias (0 , "default" , "RFC 8089 scheme (file://) (platform-aware)" , { # [cfg (not (windows))] { "file://{host}{path}" } # [cfg (windows)] { "file://{path}" } } ,) , alias ("file" , "RFC 8089 scheme (file://) with host" , "file://{host}{path}" ,) , alias ("grep+" , "grep+ scheme (grep+://)" , "grep+://{path}:{line}") , alias ("kitty" , "kitty-style RFC 8089 scheme (file://) with line number" , "file://{host}{path}#{line}" ,) , alias ("macvim" , "MacVim scheme (mvim://)" , "mvim://open?url=file://{path}&line={line}&column={column}" ,) , prioritized_alias (1 , "none" , "disable hyperlinks" , "") , alias ("textmate" , "TextMate scheme (txmt://)" , "txmt://open?url=file://{path}&line={line}&column={column}" ,) , alias ("vscode" , "VS Code scheme (vscode://)" , "vscode://file{path}:{line}:{column}" ,) , alias ("vscode-insiders" , "VS Code Insiders scheme (vscode-insiders://)" , "vscode-insiders://file{path}:{line}:{column}" ,) , alias ("vscodium" , "VSCodium scheme (vscodium://)" , "vscodium://file{path}:{line}:{column}" ,) ,] ;
};
}
