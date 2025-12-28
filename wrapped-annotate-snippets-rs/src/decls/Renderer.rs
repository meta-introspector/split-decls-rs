macro_rules! deps {
    () => {
        Report!();
        DecorStyle!();
        Stylesheet!();
    };
}

macro_rules! Renderer {
    () => {
        deps!();
        # [doc = " The [Renderer] for a [`Report`]"] # [doc = ""] # [doc = " The caller is expected to detect any relevant terminal features and configure the renderer,"] # [doc = " including"] # [doc = " - ANSI Escape code support (always outputted with [`Renderer::styled`])"] # [doc = " - Terminal width ([`Renderer::term_width`])"] # [doc = " - Unicode support ([`Renderer::decor_style`])"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use annotate_snippets::*;"] # [doc = " # use annotate_snippets::renderer::*;"] # [doc = " # use annotate_snippets::Level;"] # [doc = " let report = // ..."] # [doc = " # &[Group::with_title("] # [doc = " #     Level::ERROR"] # [doc = " #         .primary_title(\"unresolved import `baz::zed`\")"] # [doc = " #         .id(\"E0432\")"] # [doc = " # )];"] # [doc = ""] # [doc = " let renderer = Renderer::styled();"] # [doc = " let output = renderer.render(report);"] # [doc = " anstream::println!(\"{output}\");"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Renderer { anonymized_line_numbers : bool , term_width : usize , decor_style : DecorStyle , stylesheet : Stylesheet , short_message : bool , }
    };
}

Renderer!();