macro_rules! deps {
    () => {
        Renderer!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [doc = " Customize [`Renderer::styled`]"] impl Renderer { # [doc = " Override the output style for [error][crate::Level::ERROR]"] pub const fn error (mut self , style : Style) -> Self { self . stylesheet . error = style ; self } # [doc = " Override the output style for [warnings][crate::Level::WARNING]"] pub const fn warning (mut self , style : Style) -> Self { self . stylesheet . warning = style ; self } # [doc = " Override the output style for [info][crate::Level::INFO]"] pub const fn info (mut self , style : Style) -> Self { self . stylesheet . info = style ; self } # [doc = " Override the output style for [notes][crate::Level::NOTE]"] pub const fn note (mut self , style : Style) -> Self { self . stylesheet . note = style ; self } # [doc = " Override the output style for [help][crate::Level::HELP]"] pub const fn help (mut self , style : Style) -> Self { self . stylesheet . help = style ; self } # [doc = " Override the output style for line numbers in the [`Snippet`][crate::Snippet] gutter"] pub const fn line_num (mut self , style : Style) -> Self { self . stylesheet . line_num = style ; self } # [doc = " Override the output style for emphasis for the"] # [doc = " [`primary_title`][crate::Level::primary_title]"] pub const fn emphasis (mut self , style : Style) -> Self { self . stylesheet . emphasis = style ; self } # [doc = " Override the output style for [`AnnotationKind::Context`][crate::AnnotationKind::Context]"] pub const fn context (mut self , style : Style) -> Self { self . stylesheet . context = style ; self } # [doc = " Override the output style for [`Patch`][crate::Patch] additions"] pub const fn addition (mut self , style : Style) -> Self { self . stylesheet . addition = style ; self } # [doc = " Override the output style for [`Patch`][crate::Patch] removals"] pub const fn removal (mut self , style : Style) -> Self { self . stylesheet . removal = style ; self } # [doc = " Override the output style for all other text"] pub const fn none (mut self , style : Style) -> Self { self . stylesheet . none = style ; self } }
    };
}

impl_107!();